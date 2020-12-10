# HTML -> Readability -> Markdown

Intended for a pipeline like this.

```shell
content=$(osascript -e 'tell application "Safari" to return the source of front document')
url=$(osascript -e 'tell application "Safari" to return the URL of front document')

cd <reader location>/target/release/

./reader <(echo $content) $url | /usr/local/bin/pandoc -f html -t commonmark-raw_html --wrap none 
```

## To install

```shell
git clone https://github.com/dustinknopoff/raster-md
cargo build --release
```