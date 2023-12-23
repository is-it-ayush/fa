### fa.

'fa' is simple "terminal" password manager.

```sh
# initialize fa.
fa init

# basic usage
fa add meow@isitayush.dev password # add the credentials to the default store.
fa add bingus@isitayush.dev password --store bingus_store # create and/or add the credentials to the bingus store.
fa list # list all credentials for the default store.
fa list --store bingus_store # list all credentials on bingus store.
fa search meow # search for credentials beginning with 'meow' on default store
fa search bingus --store bingus_store # search for credentials beginning with 'bingus' on 'bingus_store'

# other
fa config view # display the configuration utilized by 'fa'.
fa store list # list all stores.
fa help add # get help for a specific command
fa -V # print the version
```

### installation.

wip 🚧

> sorry! i'm still working on it. i'll release it when it's done and is usable enough.

### configuration.

- 'fa' store it's configuration in `$HOME/.config/fa/config.toml`.
- in my opinion, before your run `fa ini` where it asks you a default store direcotry. you should create a directory and
do `git init`. you can then use this directory as a location of all of your stores. for now you'd
have to manually back it up to git but i'll probably automate this in future!! :3

```toml
# $HOME/.config/fa/config.toml

[store]
base_path = "/home/ayush/personal/fa/allstores/"
default_store = "dibba"

[security]
gpg_fingerprint = "ABCDEF0123456789" # you can also use the full fingerprint
```

### license.

it's MIT! if you still wanna read it checkout [LICENSE](./LICENSE.md).
