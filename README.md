

# Rust-Java-Import-Order

Re-organize the import statements of java files in a given folder.  

The imports are organized in the following order : 

- All static imports 
	 - imports starting with "java"
	 - imports starting with "javax"
	 - imports starting with "org"
	 - imports starting with "com"
	 - all other imports 


You can download the binary from [here](Rust-Java-Import-Order) and run the project as follows 

./Rust-Java-Import-Order absolute_path_of_folder

Same as what you can do in intellij by : code->Organize Imports.

So then why write a rust program to do that? Because why not!

Still a few reasons :

 1. A way to learn rust. This was my first rust program. I chose the problem statement of organizing imports of every file in a folder because : 

 - The project I wanted to perform the operation on has over 1000 java files. So I can't do that individually over each file using intellij. 
 - Writing a import organizer meant,  I need know how to read and write files and how to do custom sorting. 
 
 2. The problem wasn't a simple one but also not a difficult one. That meant it was challenging throughout but also not so much that I give up.   

The flow of the program is :

 - Go over the directories and sub-directories and find all java files. 
 - Go over each java file and get contents before and after the import statement. These will remain as is. 
 - Sort the import statements in the following order : 
	 - All static imports 
	 - imports starting with "java"
	 - imports starting with "javax"
	 - imports starting with "org"
	 - imports starting with "com"
	 - all other imports
 - Insert the before import, sorted import and after import statements. 

There are SO MANY optimizations that can be done in the code and it can be written in a much cleaner way. 