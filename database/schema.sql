-- Cleanup

DROP TABLE IF EXISTS user_data CASCADE;
DROP TABLE IF EXISTS meeting CASCADE;
DROP TABLE IF EXISTS proposed_date CASCADE;
DROP TABLE IF EXISTS proposed_date_user_votes CASCADE;
DROP TABLE IF EXISTS  meeting_comment CASCADE;

DROP TYPE IF EXISTS PROPOSED_DATE_VOTE CASCADE;

-- Declarations

CREATE TYPE PROPOSED_DATE_VOTE AS ENUM ('maybe', 'ok');

CREATE TABLE user_data (
    id UUID PRIMARY KEY,
    secret_token UUID NOT NULL,
    name VARCHAR(100)
);

CREATE TABLE meeting (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    user_data_id UUID NOT NULL, 

    CHECK (expires_at >= created_at),
    FOREIGN KEY(user_data_id)
        REFERENCES user_data(id)
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


CREATE TABLE proposed_date_user_votes (
    proposed_date_id UUID NOT NULL,
    user_data_id UUID NOT NULL,
    vote PROPOSED_DATE_VOTE NOT NULL,
    comment VARCHAR(200),

    FOREIGN KEY(proposed_date_id)
        REFERENCES proposed_date(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY(user_data_id)
        REFERENCES user_data(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE(proposed_date_id, user_data_id)
);

CREATE TABLE meeting_comment (
    id UUID PRIMARY KEY,
    user_data_id UUID NOT NULL,
    meeting_id UUID NOT NULL,
    message VARCHAR(1000) NOT NULL,
    posted_at TIMESTAMP WITH TIME ZONE NOT NULL,

    FOREIGN KEY(user_data_id)
        REFERENCES user_data(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY(meeting_id)
        REFERENCES meeting(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

