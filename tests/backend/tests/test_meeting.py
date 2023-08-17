from datetime import datetime, timezone
import uuid

import pytest
import requests

from tests.utils.actions import create_meeting_and_validate, get_meeting_info_and_validate, \
    join_meeting_and_validate, get_meeting_info, post_comment, post_comment_and_validate, \
    join_meeting
from tests.utils.models import CreateMeetingData, MeetingParticipant, PostCommentData


def test_get_noexisting_meeting_returns_404_bad_request(server_address):
    id = uuid.uuid4()
    response = get_meeting_info(server_address=server_address, id=id)
    assert response.status_code == 404


def test_post_meeting_without_content_type_header_returns_415_unsupported_media_type(server_address):
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

    new_meeting = create_meeting_and_validate(
        server_address=server_address, data=data)
    meeting_info = get_meeting_info_and_validate(
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


def test_join_nonexsistent_meeting_returns_404_not_found(server_address):
    nonexistent_meeting_id = uuid.uuid4()
    response = join_meeting(server_address=server_address,
                            meeting_id=nonexistent_meeting_id, name="User")

    assert response.status_code == 400, f"{response.status_code=}"


def test_join_meeting(server_address):
    user_name1 = "User 1"
    user_name2 = "User 2"

    meeting_data = CreateMeetingData(
        meeting_name="test name", meeting_description=None, user_name=user_name1)
    new_meeting = create_meeting_and_validate(
        server_address=server_address, data=meeting_data)

    meeting_id = new_meeting.meeting_id
    user2 = join_meeting_and_validate(server_address=server_address,
                                      meeting_id=meeting_id, name=user_name2)

    meeting_info = get_meeting_info_and_validate(
        server_address=server_address, id=new_meeting.meeting_id)

    meeting_participants = list(sorted(meeting_info.participants))
    expected_participants = list(sorted([
        MeetingParticipant(id=new_meeting.user_id, name=user_name1),
        MeetingParticipant(id=user2.id, name=user_name2),
    ]))

    assert meeting_participants == expected_participants


def test_post_comment_with_invalid_user_id_returns_401_unauthorized(server_address):
    meeting_data = CreateMeetingData(
        meeting_name="test name", meeting_description=None, user_name="user")
    new_meeting = create_meeting_and_validate(
        server_address=server_address, data=meeting_data)

    nonexistent_user_id = uuid.uuid4()
    nonexistent_user_token = uuid.uuid4()

    comment_data = PostCommentData(
        user_id=nonexistent_user_id, user_token=nonexistent_user_token, comment="Foobar")
    response = post_comment(server_address=server_address,
                            meeting_id=new_meeting.meeting_id, data=comment_data)

    assert response.status_code == 401, f"{response.status_code=}"


def test_post_comment_with_invalid_secret_token_returns_403_forbidden(server_address):
    meeting_data = CreateMeetingData(
        meeting_name="test name", meeting_description=None, user_name="user")
    new_meeting = create_meeting_and_validate(
        server_address=server_address, data=meeting_data)

    nonexistent_user_token = uuid.uuid4()

    comment_data = PostCommentData(
        user_id=new_meeting.user_id, user_token=nonexistent_user_token, comment="Foobar")
    response = post_comment(server_address=server_address,
                            meeting_id=new_meeting.meeting_id, data=comment_data)

    assert response.status_code == 403, f"{response.status_code=}"


def test_post_comment_to_nonexistent_meeting_returns_404_not_found(server_address):
    nonexistent_meeting_id = uuid.uuid4()
    nonexistent_user_id = uuid.uuid4()
    nonexistent_user_token = uuid.uuid4()

    comment_data = PostCommentData(
        user_id=nonexistent_user_id, user_token=nonexistent_user_token, comment="Foobar")
    response = post_comment(server_address=server_address,
                            meeting_id=nonexistent_meeting_id, data=comment_data)

    assert response.status_code == 404, f"{response.status_code=}"


def test_post_comment(server_address):
    # Setup
    user1_name = "user1"
    user2_name = "user2"
    user3_name = "user3"

    meeting_data = CreateMeetingData(
        meeting_name="test name", meeting_description=None, user_name=user1_name)
    new_meeting = create_meeting_and_validate(
        server_address=server_address, data=meeting_data)

    meeting_id = new_meeting.meeting_id
    user1_id = new_meeting.user_id
    user1_token = new_meeting.user_secret_token

    user2 = join_meeting_and_validate(
        server_address=server_address, meeting_id=meeting_id, name=user2_name)
    user2_id = user2.id
    user2_token = user2.secret_token

    user3 = join_meeting_and_validate(
        server_address=server_address, meeting_id=meeting_id, name=user3_name)
    user3_id = user3.id
    user3_token = user3.secret_token

    # Post comments
    comment1_text = "foo"
    comment2_text = "bar"
    comment3_text = "baz"

    time0 = datetime.now(tz=timezone.utc)

    comment1_data = PostCommentData(
        user_id=user1_id, user_token=user1_token, comment=comment1_text)
    post_comment_and_validate(
        server_address=server_address, meeting_id=meeting_id, data=comment1_data)
    time1 = datetime.now(tz=timezone.utc)

    comment2_data = PostCommentData(
        user_id=user2_id, user_token=user2_token, comment=comment2_text)
    post_comment_and_validate(
        server_address=server_address, meeting_id=meeting_id, data=comment2_data)
    time2 = datetime.now(tz=timezone.utc)

    comment3_data = PostCommentData(
        user_id=user3_id, user_token=user3_token, comment=comment3_text)
    post_comment_and_validate(
        server_address=server_address, meeting_id=meeting_id, data=comment3_data)
    time3 = datetime.now(tz=timezone.utc)

    # Validate response
    meeting_info = get_meeting_info_and_validate(
        server_address=server_address, id=meeting_id)
    comments = sorted(meeting_info.comments,
                      key=lambda comment: comment.posted_at)

    # This asserts that there are exactly 3 comments
    comment1, comment2, comment3 = comments

    assert comment1.message == comment1_text, f"{comment1=}"
    assert comment1.written_by == user1_id, f"{comment1=}"
    assert time0 <= comment1.posted_at <= time1, f"{comment1=}"

    assert comment2.message == comment2_text, f"{comment2=}"
    assert comment2.written_by == user2_id, f"{comment2=}"
    assert time1 <= comment2.posted_at <= time2, f"{comment2=}"

    assert comment3.message == comment3_text, f"{comment3=}"
    assert comment3.written_by == user3_id, f"{comment3=}"
    assert time2 <= comment3.posted_at <= time3, f"{comment3=}"
