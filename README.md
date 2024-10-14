# QRClaimIt

## Overview

QRClaimIt is an innovative web application that allows users to verify product authenticity
and register ownership via unique QR codes. This application is designed to help brands
combat counterfeiting and provide customers with a tool to claim and verify their
purchases as original.

## Features

- **Create QR**: Generates a new QR code for a product with a unique ID, secure password,
  and associated metadata.
- **Set Owner**: Allows a user to register a product under their name and email by
  claiming the QR code.
- **Delete Owner**: Permits the owner remov their ownership of a product, including their
  email associated with the QR code so the product can be give to other owner.

## Technology Stack

- **Backend Framework**: Actix Web (Rust)
- **Database**: PostgreSQL (async interaction through SQLx)
- **Logging**: `env_logger` for logging requests and application flow.
- **Middleware**: CORS is handled using `actix_cors` for cross-origin resource sharing
  support.
