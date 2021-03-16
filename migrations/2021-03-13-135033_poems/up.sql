-- Your SQL goes here-- Your SQL goes here
CREATE TABLE poems (
    id SERIAL PRIMARY KEY,
    _id VARCHAR,
    title VARCHAR NOT NULL,
    dynasty VARCHAR NULL,
    writer VARCHAR NULL,
    poemType text[],
    content text NULL,
    remark text NULL,
    translation text NULL,
    shangxi text NULL
);