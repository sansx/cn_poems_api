-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  "name" VARCHAR,
  headImageUrl VARCHAR,
  simpleIntro text,
  detailIntro json
);