# Setup
Install rust
https://www.rust-lang.org/

Install cargo - rust package manager
https://doc.rust-lang.org/cargo/getting-started/installation.html

Run the project
```
cargo run
```


cd /var/www/appassembly
ln -s ~/code/appassembly/static/dist/static static
ln -s ~/code/appassembly/templates templates

Setup db

```
sudo apt install postgresql postgresql-contrib


apt-get install libpq-dev

cargo install diesel_cli --no-default-features --features postgres


createuser --interactive --pwprompt
aasm

grant all on aasmdb to aasm

psql --user aasm --password -d aasmdb --host localhost

```

Diesel ORM
```
diesel setup
diesel migration generate create_routes
diesel migration run
```

