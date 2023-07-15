DROP TABLE IF EXISTS meetings;
DROP TABLE IF EXISTS proposed_times;

CREATE TABLE meetings (
    id  SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE proposed_times (
    meeting_id INT NOT NULL,
    date DATE NOT NULL,

    FOREIGN KEY(meeting_id)
        REFERENCES meetings(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
