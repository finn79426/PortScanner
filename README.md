# Port Scanner (Rust)

A high-performance, asynchronous CLI tool designed for network reconnaissance. It enumerates subdomains using Certificate Transparency (CT) logs and performs concurrent port scanning on resolved targets.

## Features

- **Passive Reconnaissance**: Retrieves subdomains from crt.sh (Certificate Transparency logs) to minimize direct interaction with the target initially.
- **Reachability Check**: Ensures subdomains are resolvable via DNS before port scanning.
- **Concurrent Scanning**:
  - **Subdomain Enumeration**: Processes multiple subdomains in parallel using stream buffering.
  - **Port Scanning**: Scans the top 100 most common ports for each subdomain concurrently.
- **Non-blocking I/O**: Leverages tokio and MPSC channels to manage tasks efficiently without blocking the main thread.

## Usage

```shell
cargo run --release -- github.com

>> Port scan completed in 12.611779 seconds
>> --------------------------------------------------
>> jobs.github.com
>>         5000: opened
>> 
>> api.security.github.com
>>         80: opened
>>         443: opened
>> 
>> m.communication.github.com
>>         80: opened
>>         443: opened
>> 
>> res.communication.github.com
>>         80: opened
>>         443: opened
>> 
>> skyline.github.com
>>         80: opened
>>         443: opened
>> 
>> garage.github.com
>>         22: opened
>>         443: opened
>> 
>> ws.support.github.com
>>         80: opened
>>         443: opened
>> 
>> api.mcp.github.com
>>         80: opened
>>         443: opened
>> 
>> mailing.github.com
>>         80: opened
>>         443: opened
>> 
>> raw.github.com
>>         80: opened
>>         443: opened
>> 
>> docs.github.com
>>         80: opened
>>         443: opened
>> 
>> examregistration-api.github.com
>>         80: opened
>>         443: opened
>> 
>> mona-arcade.github.com
>>         80: opened
>>         443: opened
>> 
>> edu.github.com
>>         80: opened
>>         443: opened
>> 
>> vpn-ca.iad.github.com
>>         80: opened
>>         443: opened
>> 
>> styleguide.github.com
>>         80: opened
>>         443: opened
>> 
>> gist.github.com
>>         22: opened
>>         80: opened
>>         443: opened
>> 
>> learn.github.com
>>         80: opened
>>         443: opened
>> 
>> examadmin-uat.github.com
>>         80: opened
>>         443: opened
>> 
>> community.github.com
>>         80: opened
>>         443: opened
>> 
>> help.github.com
>>         80: opened
>>         443: opened
>> 
>> slack.github.com
>>         80: opened
>>         443: opened
>> 
>> atom-installer.github.com
>>         80: opened
>>         443: opened
>> 
>> central.github.com
>>         80: opened
>>         443: opened
>> 
>> github.com
>>         22: opened
>>         80: opened
>>         443: opened
>> 
>> examregistration.github.com
>>         80: opened
>>         443: opened
>> 
>> maintainers.github.com
>>         80: opened
>>         443: opened
>> 
>> vscode-auth.github.com
>>         80: opened
>>         443: opened
>> 
>> pkg.github.com
>>         80: opened
>>         443: opened
>> 
>> t.communication.github.com
>>         80: opened
>>         443: opened
>> 
>> render.github.com
>>         22: opened
>>         80: opened
>>         443: opened
>> 
>> f.cloud.github.com
>>         80: opened
>>         443: opened
>> 
>> classroom.github.com
>>         80: opened
>>         443: opened
>> 
>> education.github.com
>>         80: opened
>>         443: opened
>> 
>> enterprise.github.com
>>         80: opened
>>         443: opened
>> 
>> communication.github.com
>> 
>> status.github.com
>>         80: opened
>>         443: opened
>> 
>> www.github.com
>>         22: opened
>>         80: opened
>>         443: opened
>> 
>> visualstudio.github.com
>>         80: opened
>>         443: opened
>> 
>> smtp.github.com
>> 
>> examregistration-uat.github.com
>>         80: opened
>>         443: opened
>> 
>> examregistration-uat-api.github.com
>>         80: opened
>>         443: opened
>> 
>> brandguide.github.com
>>         80: opened
>>         443: opened
>> 
>> examadmin.github.com
>>         80: opened
>>         443: opened
>> 
>> support.enterprise.github.com
>>         80: opened
>>         443: opened

```