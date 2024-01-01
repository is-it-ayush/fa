### fa.

A password manager that lives in your "terminal".

```sh
# initialize fa.
fa init

# add
fa add meow@isitayush.dev password # add a credential to the 'default' store.
fa add bingus@isitayush.dev password --store bingus_store #  add a credential to the 'bingus' store.
fa add isitayush secret --site isitayush.dev --tag personal # add a credential with an associated site and/or a tag.

# list
fa list # list all credentials for the default store.
fa list --store bingus_store # list all credentials on bingus store.

# search
fa search meo # search for all credentials beginning with 'meow' on 'default' store
fa search bingus --store bingus_store # search for all credentials beginning with 'bingus' on 'bingus_store'
fa search meo --filter site/isitayush.dev # search for all credentials that match both the 'meow' filter and site 'isitayush.dev' filter.
fa search '' --filter site/isitayush.dev # search for all credetentials that match 'isitayush.dev' site.
fa search '' --filter tag/personal # search for all credentials that match the tag 'personal'

# remove
fa remove meow@isitayush.dev password # remove the credential from the store.

# store usage
fs store list # list all stores
fs store add spoingus_store # create a new store 'spoingus_store'.
fs store default spoingus_store # set the default store to 'spoingus_store'.
fs store remove spoingus_store # remove the spoingus store.

# export & import
fa import --csv-file ./mock/import/sample_data.csv # import credentails from sample_data.csv (the only required condition is that the csv must contain a username & a password field. it can also contain an optional url field. all other fields would be ignored)
fa export --csv-file ./mock/export/sample_export.csv # export credentials to sample_export.csv (this creates/overwrites the file for you).

# other
fa config view # display the configuration utilized by 'fa'.
fa store add -h # get help for a specific command
fa -V # print the version
```

### installation.

- you'll need a gpg key id/fingerprint. [read this blog](https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key) till step 4.
- goto [releases](https://github.com/is-it-ayush/fa/releases) and download the latest version and extract the `fa` binary.
  - `linux`: put the binary in `/usr/bin/` and run `chmod +x /usr/bin/fa`. i'm assuming `/usr/bin` is in your `$HOME` variable. you can now, run `fa init` on your terminal.
  - `mac`: put the binary inside the directory where you store your mac binaries (it has to be on path) and grant it perms to execute itself. i'm sorry, i don't own a mac so i don't know this one for sure.
  - `windows`: put it anywhere you like, copy it's location and add it to `$PATH` environment variable and then open command prompt/terminal & run `fa init`.

> if possible! i'll make this easier for future releases.

### configuration.

- 'fa' after intialization, stores it's configuration at `$HOME/.config/fa/config.toml`.

```toml
# $HOME/.config/fa/config.toml
# this autogenerated when you run 'fa init'

[store]
base_path = "/home/ayush/personal/fa/allstores/"
default_store = "dibba"

[security]
gpg_fingerprint = "ABCDEF0123456789" # you can also use the full fingerprint
```

### todo.

- [x] use [thiserror](https://docs.rs/thiserror/latest/thiserror/index.html) and tidy up slightly.
- [x] better output and input
- [x] create, remove and default store command.
- [x] remove credential
- [x] better internal data structure instead of `Hashmap<K,V>`.
it might give me more ways to store more information. such as the
  - assocaited websites
  - tags?
  - post implementation note: `Credential` struct is meh but it works so i'm not complaining :3.
- [ ] copy to clipboard
- [ ] generator
- [ ] better ci and possibly arm64/v8 build.

### contributing.

you can always put up a pr solving any of the above todo's or anything that
you think is mising. if i think it's good. i'll merge it! just don't make
huge changes without creating a discussion first. also, it's nice if you
follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).
it's not a requirement tho but it helps.

### license.

it's MIT! if you still wanna read it checkout [LICENSE](./LICENSE.md).
