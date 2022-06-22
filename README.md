# irs-fire

## Overview

irs-fire helps you e-file Form 1099 DIV according to [Internal Revenue Service
(IRS) Publication 1220][1] for Tax Year 2021.

It's an open-source console application written in Rust that takes two
arguments: an input filename and an output filename. It reads the input file
into a string and parses that string as JSON. Then, if the output file doesn't
exist, it creates it. Otherwise, it truncates it. Finally, according to the
input data, it makes and writes records to the output file.

You can then manually upload the output file to the Filing Information Returns
Electronically (FIRE) Production or Test System.

## License

This project is licensed under [CC0][2].

## Disclaimers

**WARNING! Use irs-fire at your own risk.** While it may help your in-house
programmer/s, I, Matthew Mario Di Pasquale, made it mainly for my own needs. I
don't intend to maintain it or update it for future tax years. I'm not a
software provider, software vendor, or service provider.

I made up the tax identification numbers (TINs), transmitter control code
(TCC), names, addresses, phone numbers, email address, and payment amounts in
<i>test/file.json</i> and <i>test/file.txt</i>. Any resemblance to actual
information is entirely coincidental.

See [CC0][2], especially paragraph 4(b), for more disclaimers.

## Features

Here're some of irs-fire's features:

- It verifies that the output file will have a maximum of 99,000 A Records
- It converts certain field values to uppercase
- It automatically makes C, K, and F Records
- It verifies that each record is 750 bytes

## Limitations

Here're some of irs-fire's known limitations:

- It only supports tax years 2011 through 2021
- It only supports Form 1099-DIV
- It only supports one CF/SF state per file
- It doesn't check that the input file is correct
  - It doesn't validate field values
  - It doesn't verify that the output file will have a maximum of 1,000,000
    records
  - Etc
- It doesn't provide helpful error messages
- It doesn't compress the output file if it has more than 10,000 records

See also <i>to-do.txt</i>.

It may have unknown limitations.

## Issues

Besides the limitations listed above, irs-fire has no known issues.

It may have unknown issues.

## Instructions

### Installation

1. Install Rust
2. Download or clone this repository

### Usage

<pre><code>cargo run <i>input_file</i> <i>output_file</i></code></pre>

#### Input data

The input file must be a JSON array of objects, each representing a T, A, or B
Record.

See <i>test/file.json</i> for an example of a valid input file.

See also <i>src/main.rs</i> for which fields irs-fire looks for in the input
file.

To set vendor-related fields, hard-code them in <i>src/main.rs</i>.

#### Make a test file

1. Copy the test input file <i>test/file.json</i> to any location, eg,
   <i>~/taxes/test/2021-file.json</i>
2. Update the first object in the copy (your test input file) with your
   information
3. Make a test file at any location, eg, <i>~/taxes/test/2021-file.txt</i>, by
   running from this project's root directory:

   ```shell
   cargo run ~/taxes/test/2021-file.json ~/taxes/test/2021-file.txt
   ```

#### Make a production file

1. Copy your test input file to any location, eg,
   <i>~/taxes/2021-form-1099-div.json</i>
2. Delete line 10 from the copy (your production input file) and update the
   copy with your information
3. Make a production file at any location, eg,
   <i>~/taxes/2021-form-1099-div.txt</i>, by running from this project's root
   directory:

   ```shell
   cargo run ~/taxes/2021-form-1099-div.json ~/taxes/2021-form-1099-div.txt
   ```

### Testing

To integration test irs-fire, run:

```shell
cargo run test/file.json test/file.txt
```

and then confirm that <i>test/file.txt</i> hasn't changed, eg, by running:

```shell
git diff test/file.txt
```

## Related projects

I found these related projects by googling <i>open source irs fire</i>:

- [moov-io/irs][3]
- [Python IRS FIRE API][4]


  [1]: https://www.irs.gov/pub/irs-pdf/p1220.pdf
  [2]: https://creativecommons.org/publicdomain/zero/1.0/
  [3]: https://github.com/moov-io/irs
  [4]: https://code.launchpad.net/~tim-alwaysreformed/python-irs-fire-api
