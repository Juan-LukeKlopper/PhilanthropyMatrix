CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT,
    keplr_address TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS groups (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    image_url TEXT,
    cashout_wallet_address TEXT NOT NULL,
    primary_color TEXT,
    secondary_color TEXT,
    about_us TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS group_proposals (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    image_url TEXT,
    cashout_wallet_address TEXT NOT NULL,
    primary_color TEXT,
    secondary_color TEXT,
    about_us TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_groups (
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES groups(id) ON DELETE CASCADE,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (user_id, group_id)
);

CREATE TABLE IF NOT EXISTS donation_proposals (
    id SERIAL PRIMARY KEY,
    group_id INTEGER REFERENCES groups(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    cost INTEGER NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS donations (
    id SERIAL PRIMARY KEY,
    group_id INTEGER REFERENCES groups(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    cost INTEGER NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT,
    contract_address TEXT UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS group_donations (
    group_id INTEGER REFERENCES groups(id) ON DELETE CASCADE,
    donation_id INTEGER REFERENCES donations(id) ON DELETE CASCADE,
    PRIMARY KEY (group_id, donation_id)
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM users WHERE username = 'admin') THEN
        INSERT INTO users (username, password, created_at) VALUES ('admin', 'admin', NOW());
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM groups WHERE name = 'admin group') THEN
        INSERT INTO groups (name, cashout_wallet_address, created_at) VALUES ('admin group', 'secret1c7t2rygcxmjlj47c4sj3q6x2sxjsu6z8cue2tf', NOW());
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM user_groups WHERE user_id = (SELECT id FROM users WHERE username = 'admin') AND group_id = (SELECT id FROM groups WHERE name = 'admin group')) THEN
        INSERT INTO user_groups (user_id, group_id, is_admin) VALUES (
            (SELECT id FROM users WHERE username = 'admin'),
            (SELECT id FROM groups WHERE name = 'admin group'),
            TRUE
        );
    END IF;
END $$;