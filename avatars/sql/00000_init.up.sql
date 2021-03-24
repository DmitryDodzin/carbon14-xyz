CREATE SCHEMA IF NOT EXISTS avatars_api AUTHORIZATION avatars;

CREATE TABLE avatars_api.avatar_images
(
  user_id TEXT NOT NULL,
  platform TEXT NOT NULL,
  image_url TEXT NOT NULL,
  image_height INTEGER NOT NULL,
  image_width INTEGER NOT NULL,
  image_mimetype TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, platform)
);
