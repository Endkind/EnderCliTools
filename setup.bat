@echo off

IF NOT EXIST .venv (
    python -m venv .venv
)

CALL .venv\Scripts\activate.bat

pip install --upgrade pip
pip install pre-commit

IF EXIST requirements.txt (
    pip install -r requirements.txt
)

pre-commit install

ECHO Setup complete
