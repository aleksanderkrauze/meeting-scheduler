from dataclasses import dataclass
from datetime import datetime, date
from enum import Enum
from operator import itemgetter
from typing import Self
from uuid import UUID


@dataclass
class CreateMeetingData:
    meeting_name: str
    meeting_description: str | None
    user_name: str

    def to_json_dict(self) -> dict:
        return {
            "meeting_name": self.meeting_name,
            "meeting_description": self.meeting_description,
            "user_name": self.user_name,
        }


@dataclass
class MeetingComment:
    message: str
    written_by: UUID
    posted_at: datetime

    @staticmethod
    def from_json_dict(data: dict) -> Self:
        try:
            message, written_by, posted_at = itemgetter(
                "message", "written_by", "posted_at")(data)
            assert len(data) == 3, "excessive items in data"

            written_by = UUID(written_by)
            posted_at = datetime.fromisoformat(posted_at)

            return MeetingComment(message=message, written_by=written_by, posted_at=posted_at)
        except Exception as e:
            raise ValueError(f"failed to parse data: {data}") from e


@dataclass
class MeetingParticipant:
    id: UUID
    name: str

    @staticmethod
    def from_json_dict(data: dict) -> Self:
        try:
            id, name = itemgetter("id", "name")(data)
            assert len(data) == 2, "excessive items in data"

            id = UUID(id)

            return MeetingParticipant(id=id, name=name)
        except Exception as e:
            raise ValueError(f"failed to parse data: {data}") from e


@dataclass
class MeetingProposedDate:
    id: UUID
    date: date

    @staticmethod
    def from_json_dict(data: dict) -> Self:
        try:
            id, _date = itemgetter("id", "date")(data)
            assert len(data) == 2, "excessive items in data"

            id = UUID(id)
            _date = date.fromisoformat(_date)

            return MeetingProposedDate(id=id, date=_date)
        except Exception as e:
            raise ValueError(f"failed to parse data: {data}") from e


class Vote(Enum):
    NO = "no"
    MAYBE = "maybe"
    YES = "yes"

    @staticmethod
    def from_str(data: str) -> Self:
        match data:
            case "no":
                return Vote.NO
            case "maybe":
                return Vote.MAYBE
            case "yes":
                return Vote.YES
            case _:
                raise ValueError(f"invalid Vote: {data}")


@dataclass
class MeetingVote:
    participant_id: UUID
    date_id: UUID
    vote: Vote
    comment: str | None

    @staticmethod
    def from_json_dict(data: dict) -> Self:
        try:
            participant_id, date_id, vote, comment = itemgetter(
                "participant_id", "date_id", "vote", "comment")(data)

            participant_id = UUID(participant_id)
            date_id = UUID(date_id)
            vote = Vote.from_str(vote)

            return MeetingVote(participant_id=participant_id, date_id=date_id, vote=vote, comment=comment)
        except Exception as e:
            raise ValueError(f"failed to parse data: {data}") from e


@dataclass
class Meeting:
    name: str
    description: str | None
    created_by: UUID
    created_at: datetime
    comments: list[MeetingComment]
    participants: list[MeetingParticipant]
    proposed_dates: list[MeetingProposedDate]
    votes: list[MeetingVote]

    @staticmethod
    def from_json_dict(data: dict) -> Self:
        try:
            name, description, created_by, created_at, comments, participants, proposed_dates, votes = \
                itemgetter("name", "description", "created_by", "created_at",
                           "comments", "participants", "proposed_dates", "votes")(data)

            created_by = UUID(created_by)
            created_at = datetime.fromisoformat(created_at)
            comments = [MeetingComment.from_json_dict(c) for c in comments]
            participants = [MeetingParticipant.from_json_dict(
                p) for p in participants]
            proposed_dates = [MeetingProposedDate.from_json_dict(
                d) for d in proposed_dates]
            votes = [MeetingVote.from_json_dict(v) for v in votes]

            return Meeting(
                name=name,
                description=description,
                created_by=created_by,
                created_at=created_at,
                comments=comments,
                participants=participants,
                proposed_dates=proposed_dates,
                votes=votes,
            )
        except Exception as e:
            raise ValueError(f"failed to parse data: {data}") from e
