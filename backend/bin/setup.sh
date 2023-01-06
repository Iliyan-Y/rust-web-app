#!/bin/bash 
dropdb rust_web_app || true

createdb rust_web_app || true

psql -d rust_web_app < bin/setup.sql