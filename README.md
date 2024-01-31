## Update Checker

This is a simple rust program which is meant to be ran continuously. It polls a website every 30 seconds to 
see if any changes have been made. The program checks differences in the HTML representation of the website, so it 
may flag chances that aren't reflected in a browser. It runs an additional check to see if the text that is displayed 
on the website changes, although checking manually by opening the older file is still recommended.

## Running

### 1: Download
Go into the [binaries](binaries) folder, and download either `update_checker`, or `update_checker.exe`. The .exe file is 
for Windows computers, whilst the one without a file suffix is for MacOS & Linux. Next, put the file into a new 
directory of your choice. This will be where the program saves HTML files. 

### 2a: A Short Word on Terminals 
A terminal is an application that users interface with by typing commands, as opposed to visual elements 
(like in a GUI). Our program is a terminal program, meaning it doesn't have any visual elements, and instead is ran
by typing a command. A terminal is opened at a directory, usually the user directory, and from there, you must 
navigate to the program you want to run. For any terminal, the command `cd` should work. It stands for **c**hange 
**d**irectory, and by typing `cd "directory path"`, we can navigate into the `directory path` folder. The quotes ensure
you can `cd` into paths with spaces. On mac/linux, you can view the files and folders in your current directory by typing 
`ls` (`dir` on Windows). From there, you can choose which folder to `cd` into.

### 2b: Opening the Terminal 
Next, open your terminal application (launchpad > terminal on macOS), (cmd on Windows -- if you have trouble 
finding, press the Windows key + r and just type in cmd & hit enter). Now, we have to `cd` into the directory where we
saved the program. Assuming it's in a folder titled `update_checker_dir` within your downloads folder, you should be 
able to type `cd "Downloads/update_checker_dir"` to move into the directory with the program. 

### 3a: Arguments
Terminal applications often have arguments, i.e., values passed into the program in order for the program to run a more
specific task. The directory after `cd` is an argument. This program can take 1 or 2 arguments. The first one is a 
URL [(default)](https://policy.cornell.edu/policy-library/interim-expressive-activity-policy), whilst the second is the 
frequency at which the program will poll the website (default: 30). 

### 3b: Running the program
If you are comfortable with the defaults, simply type either `./update_checker` or `./update_checker.exe` depending on 
your operating system. If you wish to modify the arguments, simply type `./update_checker https://website-example.
com 30` (replacing the arguments with your desired arguments) and hit enter. 

That's it! Keep in mind that you have to have the program running in the background, or else it won't keep track of 
the website, but you can minimize the terminal window as long as you don't completely exit out of it (that would 
kill the program).
