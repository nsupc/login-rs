# Login-rs
A very basic login script for maintaining nations on [NationStates](https://www.nationstates.net)
## Features
- Easy to use
- Keeps nations alive
- Doesn't get you API banned*
## Instructions
1. Download the binary for your operating system.
2. Create a file called "nations.txt" in the same folder as the application, and add your nations and their passwords to the file in the following format:
    ```
       Nation 1,Password 1
       nation_2,Password 2
       etc
       etc
    ```
3. Run the binary (double click it)
4. Submit your User-Agent
5. That's it. Don't close the window until the script terminates.
## Running via CLI or a Batch/Shell Script
- You can set your User-Agent (-u), adjust the script's rate limit (-s), and specify a different file with nations (-p) via CLI arguments.
- You can use this to automate your logins via tools such as Windows Task Scheduler or Linux's cron
- A basic batch script would look like this ``login -u UPC -s 40 -p "D:\NS\nations.txt"``
- Run "login -h" for more information


\*This script will not exceed the NationStates rate limit on its own, but use caution when running multiple scripts concurrently. You can lower this script's rate limit via command line arguments. 