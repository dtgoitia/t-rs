# Toggl CLI

CLI client to interact with the Toggl API. It

## Usage

```shell
nix run
```

## Configuration

The configuration file must be at:

```
$HOME/.config/t/config.jsonc
```

with the following schema:

```jsonc
{
  "projects": [
    {
      "id": 123345,
      "name": "Best project ever",
      "entries": ["General", "Training", "Mentoring"],
      "workplace_id": 54321,
    },
    // ...
  ],
}
```

## Credentials

The credentials file must be at:

```
$HOME/.config/t/credentials.jsonc
```

with the following schema:

```jsonc
{
  "toggl_api_token": "abcd1234",
}
```

## Development

```shell
nix develop
```
