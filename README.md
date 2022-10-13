# Page Status

Light-weight service designed to poll web pages for their status codes and
write the result to disk.

I made page-status to use with a simple xmobar status script to alert me on my
local desktop if one of my sites was down. It makes no attempt to do monitoring
or alerting beyond recording the status codes from the configured websites. It
is a simple single purpose tool intended to provide data useful for some other
monitoring or alerting solution.

## Installation

Look for pre-built binaries in the
[releases](https://github.com/julianandrews/page-status/releases).

The `.deb` package should install a systemd service. Before that will be useful
you'll need to edit the configuration and restart the service with:

    systemctl restart page-status.service

## Configuration

Configuration is done by editing `/etc/page-status/config.toml`. See that file
or `pkg/config.toml` in the repository for an sample configuration.

## Usage

Simple running

    page-status

will cause page-status to read your configuration and launch. I personally use
a systemd service to start page-status and record logs.

By default output is written to `/var/cache/page-status` with one file per
page. Files will either contain the http status code of the last request, or
the output of the last error to occur for that page.

## Contributing

Pull requests are welcome. For non-trivial changes, please open an issue to
discuss the change before starting.
