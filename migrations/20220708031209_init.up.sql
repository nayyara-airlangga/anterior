CREATE SCHEMA IF NOT EXISTS "posterior";

CREATE TABLE "posterior"."users" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "email" varchar(255) UNIQUE NOT NULL,
  "username" varchar(255) UNIQUE NOT NULL,
  "name" varchar(255) NOT NULL,
  "password" varchar(255) NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE "posterior"."posts" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "title" varchar(255) NOT NULL,
  "headline" varchar(1000) NOT NULL,
  "slug" varchar(255) UNIQUE NOT NULL,
  "content" text NOT NULL,
  "published" boolean NOT NULL DEFAULT false,
  "created_at" timestamptz NOT NULL DEFAULT (now()),
  "edited_at" timestamptz NOT NULL DEFAULT (now()),
  "published_at" timestamptz,
  "author_id" int NOT NULL
);

CREATE TABLE "posterior"."comments" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "comment" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now()),
  "edited_at" timestamptz NOT NULL DEFAULT (now()),
  "post_id" int NOT NULL,
  "author_id" int NOT NULL,
  "parent_id" int
);

CREATE INDEX ON "posterior"."users" ("id");

CREATE INDEX ON "posterior"."users" ("username");

CREATE INDEX ON "posterior"."users" ("name");

CREATE INDEX ON "posterior"."posts" ("id");

CREATE INDEX ON "posterior"."posts" ("title");

CREATE INDEX ON "posterior"."posts" ("slug");

CREATE INDEX ON "posterior"."comments" ("id");

ALTER TABLE "posterior"."posts" ADD FOREIGN KEY ("author_id") REFERENCES "posterior"."users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "posterior"."comments" ADD FOREIGN KEY ("author_id") REFERENCES "posterior"."users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "posterior"."comments" ADD FOREIGN KEY ("post_id") REFERENCES "posterior"."posts" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "posterior"."comments" ADD FOREIGN KEY ("parent_id") REFERENCES "posterior"."comments" ("id") ON DELETE SET NULL ON UPDATE CASCADE;
