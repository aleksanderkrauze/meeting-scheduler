from datetime import datetime, timezone
import uuid

import pytest
import requests

from tests.utils.actions import create_meeting, get_meeting_info, join_meeting
from tests.utils.models import CreateMeetingData, MeetingParticipant


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

    new_meeting = create_meeting(server_address=server_address, data=data)
    meeting_info = get_meeting_info(
        server_address=server_address, id=new_meeting.meeting_id)

    assert meeting_info.name == data.meeting_name
    assert meeting_info.description == data.meeting_description
    assert len(meeting_info.comments) == 0
    assert len(meeting_info.proposed_dates) == 0
    assert len(meeting_info.votes) == 0

    assert len(meeting_info.participants) == 1
    creator = meeting_info.participants[0]
    assert meeting_info.created_by == creator.id
    assert creator.name == data.user_name

    timestamp_after_request = datetime.now(tz=timezone.utc)
    assert meeting_info.created_at.tzinfo is not None
    assert timestamp_before_request < meeting_info.created_at < timestamp_after_request


def test_join_meeting(server_address):
    user_name1 = "User 1"
    user_name2 = "User 2"

    meeting_data = CreateMeetingData(
        meeting_name="test name", meeting_description=None, user_name=user_name1)
    new_meeting = create_meeting(
        server_address=server_address, data=meeting_data)

    meeting_id = new_meeting.meeting_id
    user2 = join_meeting(server_address=server_address,
                         meeting_id=meeting_id, name=user_name2)

    meeting_info = get_meeting_info(
        server_address=server_address, id=new_meeting.meeting_id)

    meeting_participants = list(sorted(meeting_info.participants))
    expected_participants = list(sorted([
        MeetingParticipant(id=new_meeting.user_id, name=user_name1),
        MeetingParticipant(id=user2.id, name=user_name2),
    ]))

    assert meeting_participants == expected_participants
