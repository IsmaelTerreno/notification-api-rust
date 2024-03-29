-- This file should undo anything in `up.sql`
ALTER TABLE notification
    DROP COLUMN user_id;
