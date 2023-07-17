INSERT INTO user_data(id, secret_token, name) VALUES
    ('0d733785-9e90-4f98-aa3a-687f7fc17634', 'f5fc6cb8-3055-4ced-b31d-2fddc61258cb', 'Alice'),
    ('26eacae9-2bc2-4611-8839-c9a3e9086c10', '5770d8cf-22a7-4b25-b108-75ea048143a3', 'Bob'),
    ('73fd1be4-539b-4d60-9bd7-cef6eb5bf721', '47cf8fa8-8b27-47da-9966-2f731efe10d5', 'Charlie'),
    ('e6d073dd-755d-40e4-8cc7-f5ffe88edb49', 'bf31a94e-0c66-4b90-ac6e-e617c2b3da32', 'Derek');

INSERT INTO meeting(id, name, description, created_at, expires_at, user_data_id) VALUES
    (
        '0dbee40e-95d7-4d70-ac16-f3501977940b',
        'Meeting 1',
        'Meeting where we will be planning to plan.',
        '2023-07-1 10:00:00+00',
        '2023-07-15 10:00:00+00',
        '0d733785-9e90-4f98-aa3a-687f7fc17634'
    );

INSERT INTO proposed_date(id, meeting_id, date) VALUES
    ('ce5e7889-0bb1-4ff0-8efb-3697a3328590', '0dbee40e-95d7-4d70-ac16-f3501977940b', '2023-07-10'),
    ('056a56b8-8080-49ad-b16d-7b8325215157', '0dbee40e-95d7-4d70-ac16-f3501977940b', '2023-07-11'),
    ('6fad22d3-f777-4b00-a6df-1912bb4d7b79', '0dbee40e-95d7-4d70-ac16-f3501977940b', '2023-07-12'),
    ('29cce3e4-d518-41cc-bd6e-648f74d8379d', '0dbee40e-95d7-4d70-ac16-f3501977940b', '2023-07-13'),
    ('9e52126f-5dd8-4a1c-9b97-8f5eb015fb80', '0dbee40e-95d7-4d70-ac16-f3501977940b', '2023-07-14');

INSERT INTO proposed_date_user_votes(proposed_date_id, user_data_id, vote, comment) VALUES
    ('ce5e7889-0bb1-4ff0-8efb-3697a3328590', '0d733785-9e90-4f98-aa3a-687f7fc17634', 'ok', NULL),
    ('056a56b8-8080-49ad-b16d-7b8325215157', '0d733785-9e90-4f98-aa3a-687f7fc17634', 'ok', NULL),
    ('29cce3e4-d518-41cc-bd6e-648f74d8379d', '0d733785-9e90-4f98-aa3a-687f7fc17634', 'ok', NULL),
    ('056a56b8-8080-49ad-b16d-7b8325215157', '26eacae9-2bc2-4611-8839-c9a3e9086c10', 'ok', NULL),
    ('6fad22d3-f777-4b00-a6df-1912bb4d7b79', '26eacae9-2bc2-4611-8839-c9a3e9086c10', 'ok', NULL),
    ('29cce3e4-d518-41cc-bd6e-648f74d8379d', '73fd1be4-539b-4d60-9bd7-cef6eb5bf721', 'ok', NULL),
    ('9e52126f-5dd8-4a1c-9b97-8f5eb015fb80', '73fd1be4-539b-4d60-9bd7-cef6eb5bf721', 'no', 'I have another meeting this day'),
    ('056a56b8-8080-49ad-b16d-7b8325215157', 'e6d073dd-755d-40e4-8cc7-f5ffe88edb49', 'maybe', 'foobar'),
    ('6fad22d3-f777-4b00-a6df-1912bb4d7b79', 'e6d073dd-755d-40e4-8cc7-f5ffe88edb49', 'ok', NULL);

INSERT INTO meeting_comment(id, user_data_id, meeting_id, message, posted_at) VALUES
    (
        'dafb3e95-525a-4b8a-b1d6-778b7799dda4',
        '0d733785-9e90-4f98-aa3a-687f7fc17634',
        '0dbee40e-95d7-4d70-ac16-f3501977940b',
        'Hey guys, lets plan our meeting! 😊',
        '2023-07-1 10:00:05+00'
    ),
    (
        '5b9721f9-a443-46ff-af29-a43a9162e675',
        '26eacae9-2bc2-4611-8839-c9a3e9086c10',
        '0dbee40e-95d7-4d70-ac16-f3501977940b',
        'I''ve been waiting so long for this...',
        '2023-07-1 10:11:00+00'
    ),
    (
        'b8fcbd92-91d2-4856-8dcd-feb97cd9586f',
        'e6d073dd-755d-40e4-8cc7-f5ffe88edb49',
        '0dbee40e-95d7-4d70-ac16-f3501977940b',
        'Sorry for posting that late. I don''t think I will be able to come.',
        '2023-07-5 14:00:00+00'
    );
