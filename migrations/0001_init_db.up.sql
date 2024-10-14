CREATE TABLE qr (
    id VARCHAR(36) NOT NULL,                       -- UUID as string
    email VARCHAR(255) NULL,                       -- Optional email
    password VARCHAR(10) NOT NULL,                 -- Password with a fixed length of 10 characters
    creation_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    redemption_date TIMESTAMP WITH TIME ZONE NULL, -- Date of redemption, optional
    PRIMARY KEY (id)
);
