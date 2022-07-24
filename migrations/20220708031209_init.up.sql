CREATE SCHEMA IF NOT EXISTS "osiris";

CREATE TABLE IF NOT EXISTS "osiris"."users" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "email" varchar(255) UNIQUE NOT NULL,
  "username" varchar(255) UNIQUE NOT NULL,
  "name" varchar(255) NOT NULL,
  "password" varchar(255) NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE IF NOT EXISTS "osiris"."posts" (
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

CREATE TABLE IF NOT EXISTS "osiris"."comments" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "comment" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now()),
  "edited_at" timestamptz NOT NULL DEFAULT (now()),
  "post_id" int NOT NULL,
  "author_id" int NOT NULL,
  "parent_id" int
);

CREATE INDEX IF NOT EXISTS users_id_idx ON "osiris"."users" ("id");

CREATE INDEX IF NOT EXISTS users_username_idx ON "osiris"."users" ("username");

CREATE INDEX IF NOT EXISTS users_name_idx ON "osiris"."users" ("name");

CREATE INDEX IF NOT EXISTS posts_id_idx ON "osiris"."posts" ("id");

CREATE INDEX IF NOT EXISTS posts_title_idx ON "osiris"."posts" ("title");

CREATE INDEX IF NOT EXISTS posts_slug_idx ON "osiris"."posts" ("slug");

CREATE INDEX IF NOT EXISTS comments_id_idx ON "osiris"."comments" ("id");

ALTER TABLE "osiris"."posts" ADD FOREIGN KEY ("author_id") REFERENCES "osiris"."users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "osiris"."comments" ADD FOREIGN KEY ("author_id") REFERENCES "osiris"."users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "osiris"."comments" ADD FOREIGN KEY ("post_id") REFERENCES "osiris"."posts" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "osiris"."comments" ADD FOREIGN KEY ("parent_id") REFERENCES "osiris"."comments" ("id") ON DELETE SET NULL ON UPDATE CASCADE;
