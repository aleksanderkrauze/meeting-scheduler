from uuid import UUID

import requests

from tests.utils.models import CreateMeetingData, CreateMeetingResponse, Meeting, \
    JoinMeetingResponse, JoinMeetingData


def create_meeting(server_address: str, data: CreateMeetingData) -> CreateMeetingResponse:
    """Creates new meetng using provided `CreateMeetingData` and validates responses"""

    url = f"http://{server_address}/meeting"
    post_request = requests.post(url=url, json=data.to_json_dict())
    assert post_request.status_code == 201

    response_data = post_request.json()
    return CreateMeetingResponse.from_json_dict(response_data)


def get_meeting_info(server_address: str, id: UUID):
    """Gets meeting info and validates responses"""

    url = f"http://{server_address}/meeting/{id}"
    get_request = requests.get(url=url)
    assert get_request.status_code == 200

    response_data = get_request.json()
    return Meeting.from_json_dict(response_data)


def join_meeting(server_address: str, meeting_id: UUID, name: str) -> JoinMeetingResponse:
    """Adds new participant to meeting with `meeting_id` with given `name` and validates responses"""

    url = f"http://{server_address}/meeting/{id}/join"
    data = JoinMeetingData(name=name)
    post_request = requests.post(url=url, json=data.to_json_dict())
    assert post_request.status_code == 201

    response_data = post_request.json()
    return JoinMeetingResponse.from_json_dict(response_data)
