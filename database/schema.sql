-- Cleanup

DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS meeting CASCADE;
DROP TABLE IF EXISTS proposed_date CASCADE;
DROP TABLE IF EXISTS meeting_participants CASCADE;
DROP TABLE IF EXISTS proposed_date_user_votes CASCADE;
DROP TABLE IF EXISTS meeting_comment CASCADE;

DROP TYPE IF EXISTS proposed_date_vote CASCADE;

-- Declarations

CREATE TYPE proposed_date_vote AS ENUM ('no', 'maybe', 'ok');

CREATE TABLE users (
    id UUID PRIMARY KEY,
    secret_token UUID NOT NULL,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE meeting (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    user_id UUID NOT NULL, 

    CHECK (expires_at >= created_at),
    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE proposed_date (
    id UUID PRIMARY KEY,
    meeting_id UUID NOT NULL,
    date DATE NOT NULL,

    FOREIGN KEY(meeting_id)
        REFERENCES meeting(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE(meeting_id, date)
);

CREATE TABLE meeting_participants (
    user_id UUID NOT NULL UNIQUE,
    meeting_id UUID NOT NULL,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY(meeting_id)
        REFERENCES meeting(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);


CREATE TABLE proposed_date_user_votes (
    proposed_date_id UUID NOT NULL,
    user_id UUID NOT NULL,
    vote proposed_date_vote NOT NULL,
    comment VARCHAR(200),

    FOREIGN KEY(proposed_date_id)
        REFERENCES proposed_date(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY(user_id)
        REFERENCES meeting_participants(user_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE(proposed_date_id, user_id)
);

CREATE TABLE meeting_comment (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    meeting_id UUID NOT NULL,
    message VARCHAR(1000) NOT NULL,
    posted_at TIMESTAMP WITH TIME ZONE NOT NULL,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY(meeting_id)
        REFERENCES meeting(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

