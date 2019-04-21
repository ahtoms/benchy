# Benchy

## What is it?

Benchy is a quick and dirty benchmark hoster. Feed it a configuration file and it will accept submissions, execute them and then store the results in a local sqlite database. Once a submission is received it will send it to the test runner to be executed.


## Okay, how do I use it?

The expected json file expects configuration for the server component, where the static files to be hosted are located as well as links to the scripts it will use for each submission it receives.

Here is an example of the following script:
```
{
    "name" : "My Project Name",
    "port" : 9001,
    "root" : "~/web_files"
    "prepare_cmd" : "~/project_test/prepare.sh",
    "execute_cmd" : "~/project_test/execute.sh",
    "cleanup_cmd" : "~/project_test/cleanup.sh",
    "path" : "~/project_test/"
}
```

To run benchy, once compiled you can simply run the program like any other application and specify the json object as the first and only argument.

```
$ ./benchy config.json
```

## Sure, but how does it get the data?

Although in the future we will have a mechanism to allow for data interpretation script, for now your execute script **must** output JSON. This data will be stored in the sqlite database and readily retrieved for anyone requesting the submission list on the frontend.


## What's the API?

Only necessary information has been exposed. The following endpoints are available:

* `/info` - Retrieves information about the setup. (root will be removed soon)
```
{
    "name" : "My Project Name",
    "root" : "~/web_files",
    "test" : [ "test0", "test1", ... ]
}
```


* `/submission` - Returns an array of submissions, containing an identifier and execution data

```
[
    {
        "sub_id" : 0,
        "ident" : "Jeffyjeff",
        "test" : "[ ... ]"
    }
]
```


* `/submit` - Allows for users to submit a file, requires an identifier (String) and base64 encoded .zip file.
    
```
{
    "ident" : "Jeffyjeff",
    "data" : "2779abf23na..."
}
```


## Is there more to do?

Heck yeah! We still need to extend the routes, refactor code and provide greater functionality and flexibility for this app. There is still a lot of testing and tests to be done with this program before it should be used in serious production manner.




