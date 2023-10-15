# Problem Description:
Develop a command line application in Rust that allows incremental backups from a source directory to a target directory. The initial backup will copy the entire contents of the source directory to the target directory. Subsequent backups will only copy new or changed files since the last backup.

## To Do

- [x] The application must accept the source directory and target directory as command line arguments.
- [x] You must check if the source directory and target directory exist.
- [x] If you are running the application for the first time, you must make a complete copy from the source directory to the target directory 
- [ ] On subsequent runs, you should only copy new or changed files since the last backup.
- [ ] The application must keep a record of the last backup performed, so it must track the date and time of the last successful backup.
- [ ] It should handle errors and provide clear messages to the user in case of problems.