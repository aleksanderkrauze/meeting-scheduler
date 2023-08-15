from uuid import UUID

import requests

from tests.utils.models import CreateMeetingData, CreateMeetingResponse, Meeting, \
    JoinMeetingResponse, JoinMeetingData, PostCommentData


def create_meeting(server_address: str, data: CreateMeetingData) -> requests.Response:
    """Creates new meetng using provided `CreateMeetingData`"""

    url = f"http://{server_address}/meeting"
    return requests.post(url=url, json=data.to_json_dict())


def create_meeting_and_validate(server_address: str, data: CreateMeetingData) -> CreateMeetingResponse:
    """Creates new meetng using provided `CreateMeetingData` and validates response"""

    response = create_meeting(server_address=server_address, data=data)
    assert response.status_code == 201, f"{response.status_code=}"

    response_data = response.json()
    return CreateMeetingResponse.from_json_dict(response_data)


def get_meeting_info(server_address: str, id: UUID) -> requests.Response:
    """Gets meeting info"""

    url = f"http://{server_address}/meeting/{id}"
    return requests.get(url=url)


def get_meeting_info_and_validate(server_address: str, id: UUID):
    """Gets meeting info and validates response"""

    response = get_meeting_info(server_address=server_address, id=id)
    assert response.status_code == 200, f"{response.status_code=}"

    response_data = response.json()
    return Meeting.from_json_dict(response_data)


def join_meeting(server_address: str, meeting_id: UUID, name: str) -> requests.Response:
    """Adds new participant to meeting with `meeting_id` with given `name`"""

    url = f"http://{server_address}/meeting/{meeting_id}/join"
    data = JoinMeetingData(name=name)
    return requests.post(url=url, json=data.to_json_dict())


def join_meeting_and_validate(server_address: str, meeting_id: UUID, name: str) -> JoinMeetingResponse:
    """Adds new participant to meeting with `meeting_id` with given `name` and validates response"""

    response = join_meeting(server_address=server_address,
                            meeting_id=meeting_id, name=name)
    assert response.status_code == 201, f"{response.status_code=}"

    response_data = response.json()
    return JoinMeetingResponse.from_json_dict(response_data)


def post_comment(server_address: str, meeting_id: UUID, data: PostCommentData) -> requests.Response:
    """Posts comment as given user"""

    url = f"http://{server_address}/meeting/{meeting_id}/comment"
    return requests.post(url=url, json=data.to_json_dict())


def post_comment_and_validate(server_address: str, meeting_id: UUID, data: PostCommentData) -> requests.Response:
    """Posts comment as given user and validates response"""

    response = post_comment(server_address=server_address,
                            meeting_id=meeting_id, data=data)
    assert response.status_code == 201, f"{response.status_code=}"
    assert len(response.content) == 0, f"{response.content=}"
