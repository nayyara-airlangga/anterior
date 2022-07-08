CREATE TABLE "users" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "email" varchar(255) UNIQUE NOT NULL,
  "username" varchar(255) UNIQUE NOT NULL,
  "name" varchar(255) NOT NULL,
  "password" varchar(255) NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE "posts" (
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

CREATE TABLE "comments" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "comment" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now()),
  "edited_at" timestamptz NOT NULL DEFAULT (now()),
  "post_id" int NOT NULL,
  "author_id" int NOT NULL,
  "parent_id" int
);

CREATE INDEX ON "users" ("id");

CREATE INDEX ON "users" ("username");

CREATE INDEX ON "users" ("name");

CREATE INDEX ON "posts" ("id");

CREATE INDEX ON "posts" ("title");

CREATE INDEX ON "posts" ("slug");

CREATE INDEX ON "comments" ("id");

ALTER TABLE "posts" ADD FOREIGN KEY ("author_id") REFERENCES "users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "comments" ADD FOREIGN KEY ("author_id") REFERENCES "users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "comments" ADD FOREIGN KEY ("post_id") REFERENCES "posts" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "comments" ADD FOREIGN KEY ("parent_id") REFERENCES "comments" ("id") ON DELETE SET NULL ON UPDATE CASCADE;
