from datetime import datetime, timezone
import uuid

import pytest
import requests

from tests.utils.models import CreateMeetingData, Meeting


def test_get_noexisting_meeting_returns_404(server_address):
    id = uuid.uuid4()
    url = f"http://{server_address}/meeting/{id}"

    r = requests.get(url=url)
    assert r.status_code == 404


def test_post_meeting_without_content_type_header_returns_415(server_address):
    url = f"http://{server_address}/meeting"
    r = requests.post(url=url, data="")

    assert r.status_code == 415


@pytest.mark.parametrize("data", [
    CreateMeetingData(meeting_name="Some name",
                      meeting_description="Some description", user_name="Some user"),
    CreateMeetingData(meeting_name="Some name",
                      meeting_description=None, user_name="Some user"),
])
def test_create_meeting_success_and_meeting_response_is_correct(server_address, data: CreateMeetingData):
    timestamp_before_request = datetime.now(tz=timezone.utc)

    url = f"http://{server_address}/meeting"
    post_request = requests.post(url=url, json=data.to_json_dict())
    assert post_request.status_code == 201

    response_data = post_request.json()
    user_id = response_data["user_id"]
    user_secret_token = response_data["user_secret_token"]
    meeting_id = response_data["meeting_id"]

    # assert that data is proper UUIDs
    _ = uuid.UUID(user_id)
    _ = uuid.UUID(user_secret_token)
    _ = uuid.UUID(meeting_id)

    # assert there is no additional data
    assert len(response_data) == 3

    url = f"http://{server_address}/meeting/{meeting_id}"
    get_request = requests.get(url=url)
    assert get_request.status_code == 200

    response_data = get_request.json()
    meeting = Meeting.from_json_dict(response_data)

    assert meeting.name == data.meeting_name
    assert meeting.description == data.meeting_description
    assert len(meeting.comments) == 0
    assert len(meeting.proposed_dates) == 0
    assert len(meeting.votes) == 0

    assert len(meeting.participants) == 1
    creator = meeting.participants[0]
    assert meeting.created_by == creator.id
    assert creator.name == data.user_name

    timestamp_after_request = datetime.now(tz=timezone.utc)
    assert meeting.created_at.tzinfo is not None
    assert timestamp_before_request < meeting.created_at < timestamp_after_request
