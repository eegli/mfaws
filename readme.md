# mfaws

A cross-platform CLI tool to easily manage AWS credentials for MFA-enabled accounts. **mfaws** talks to the [AWS Security Token Service API](https://docs.aws.amazon.com/STS/latest/APIReference/welcome.html) and allows you to obtain temporary credentials using your AWS access key, AWS secret key and MFA device.

Supported STS operations:

- AssumeRole
- GetSessionToken

**mfaws** is heavily inspired by [`aws-mfa`](https://github.com/broamski/aws-mfa), with two key differences:

- Assume multiple short-term profiles for a single long-term profile
- A single native binary - no dependency on Python

If you're migrating and curious, read the section about the differences: [Migrating from `aws-mfa`: What's different?](#migrating-from-aws-mfa-whats-different)

## Installation

**mfaws** is available for Windows, MacOs and Linux.

- Via cargo:

```shell
cargo install mfaws
```

- From GitHub:

1. Download the latest binary from the [release page](https://github.com/eegli/mfaws/releases/latest)
2. Extract it
3. Add it to your `PATH`

## Credentials File

Let's assume you have the following AWS credentials file in `~/.aws/credentials`. It has a single _long-term_ profile, `dev`, which can be used to generate _short-term_ profiles. Short-term profiles are identified by the `-short-term` suffix (or a custom one that you provide). Short-term profiles are generated automatically and should not be fiddled with manually.

```ini
[dev]
aws_access_key_id=AKMB6EHIO4AB9FRYI37
aws_secret_access_key=qAnFonnuEUqp
```

- You can set `aws_mfa_device=[MFA DEVICE ARN]` in your AWS credentials profile so you don't have to pass it as a flag every time
- If you don't specify a profile name with `--profile`, the app looks for the profile named `default`

## Basic Usage

- Get a **temporary session token** for profile `dev`:

```shell
mfaws session-token --profile dev --device arn:aws:iam::3687901:mfa/my-mfa-device
```

**mfaws** automatically generates and adds the following short-term profile to your AWS credentials file:

```ini
[dev]
aws_access_key_id=AKMB6EHIO4AB9FRYI37
aws_secret_access_key=qAnFonnuEUqp

[dev-short-term]
expiration=2023-04-05T21:57:52Z
aws_access_key_id=ASIAVMB6EHIOYTGUOE7T
aws_secret_access_key=E6HGxHXHb2hqP3az+UMThIjWGVsdKH3pG1h67FxR
aws_session_token=IQoJb3JpZ2luX2VjECoaCXVzLWVhc3QtMSJHMEUCIDSFI50`

```

- **Assume a role** for profile `dev`:

```shell
mfaws assume-role --profile dev --role-arn arn:aws:iam::6823sdf5:role/admin --device arn:aws:iam::3687901:mfa/my-mfa-device
```

Now, your AWS config file looks like this:

```ini
[dev]
aws_access_key_id=AKMB6EHIO4AB9FRYI37
aws_secret_access_key=qAnFonnuEUqp

[dev_6823sdf5-role-admin-mfa-user_short-term]
assumed_role_arn=arn:aws:iam::6823sdf5:role/admin
assumed_role_id=AROAZ5XVG55QR3R2:mfa-user
expiration=2023-04-05T11:02:10Z
aws_access_key_id=ASINQT6HE6ZCS
aws_secret_access_key=iqVoWOI8+l6WVBn8pdCc/JxJ6
aws_session_token=IQoJb3JpZ2luXS4VhObxKg6p79Pm38C4ahGqcGKw==
```

Whenever you run an operation, **mfaws** checks your existing short-term profiles to see if there is still a valid (i.e., not yet expired) profile around. If that is the case, the operation is gracefully aborted and you'll be notified.

## Shell Aliases

I recommended creating bash aliases for any of these operations and then set the [`AWS_PROFILE` environment variable](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-using-profiles) to the name of the genreated profile.

E.g., for bash:

```shell
alias mfa-admin="mfaws assume-role --profile dev --role-arn arn:aws:iam::6823sdf5:role/admin && export AWS_PROFILE=default_6823sdf5-role-admin-mfa-user_short-term"
```

You might want to run it manually the first time to see what name is generated for your short-term profile. It's a combination of the assumed role and role name.

## Full Usage

In your terminal: Run:

```
mfaws help
```

**mfaws** allows you to customize many things, including the duration of the temporary credentials, the short-term suffix that is used to generate short-term profiles or the path to the credentials file. Many values can also be read from the corresponding environment variables.

```shell
Usage: mfaws [OPTIONS] <COMMAND>

Commands:
  assume-role
          Temporary credentials for an assumed AWS IAM Role
  session-token
          Temporary credentials for an AWS IAM user
  help
          Print this message or the help of the given subcommand(s)

Options:
      --profile <PROFILE_NAME>
          The AWS credentials profile to use

          [env: AWS_PROFILE=]
          [default: default]

      --device <MFA_DEVICE>
          The MFA Device ARN. This value can also be provided via the ~/.aws/credentials variable 'aws_mfa_device'

          [env: MFA_DEVICE=]

      --duration <DURATION>
          The duration, in seconds, for which the temporary credentials should remain valid. Defaults to 43200 (12 hours) for session tokens and 3600 (one hour) when assuming a role

          [env: MFA_STS_DURATION=]

      --short-term-suffix <SHORT_TERM_SUFFIX>
          To identify the auto-generated short-term credential profile by [<profile_name>-SHORT_TERM_SUFFIX]

          [default: short-term]

      --credentials <CREDENTIALS>
          Location of the AWS credentials file. Can be a relative path from your home directory or an absolute path to the file

          [env: AWS_SHARED_CREDENTIALS_FILE=]
          [default: .aws/credentials]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Roadmap and Todos

- As of now, all requests to STS hit `us-east-1` instead of a regional endpoint. Millisecond latency does not really matter for this tool, but it'd be nice being able to specify a custom regional endpoint (or read it from `~/.aws/conf`)

## Migrating from `aws-mfa`: What's different?

1. By default, all profiles are considered long-term profiles unless they end with the short term suffix set by `--short-term-suffix [SHUFFIX]`. There is no such thing as an _explicit_ long-term suffix (hence, also no `--long-term-suffix` flag)
2. Unlike `aws-mfa`, where actions (AssumeRole/GetSessionToken) are implicitly given by the presence of the `--assume-role` flag, **mfaws** has dedicated sub-commands for each operation
3. `--assume-role` is `--role-arn`
4. `--role-session-name [NAME]` does not use the [login name of your user](https://docs.python.org/3/library/getpass.html) by default but the static string `mfa-user`

## Contributing and Notes

At this point, **mfaws** is merely out of its "alpha" stage and, although stable, lacks a lot cruical properties. There's no testing strategy, no integration/e2e tests 😢.

I'm still a complete beginner with Rust, and suggestions on how to improve this project and make things prettier are very welcome! Of course, I'm also very happy for general feedback, bugfixes and feature ideas.

## Acknowledgements

- [broamski](https://github.com/broamski) for the MIT license of [`aws-mfa`](https://github.com/broamski/aws-mfa). The general idea for this tool and much of the help command descriptions were stolen from his work.
