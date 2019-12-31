# actionapp
Aplikacja jest napisana w jezyku Rust.
Instrukcja instalacji rust-lang: https://www.tutorialspoint.com/rust/rust_environment_setup.htm
Kolejny krok to: aplikacja dzialala prawidlowo, gdy rust jest w wersji nightly
wersje nightly mozemy ustawic dla naszej maszyny wpisujac w konsoli komende: rustup default nightly
aktualna wersje mozemy sprawdzic komenda: rustc --version
ten projekt zostal utworzony przy pomocy wersji: rustc 1.42.0-nightly (da3629b05 2019-12-29)

Nastepnie pobieramy repozytorium z github.
W pliku .env w glównym folderze projektu ustawiamy adres bazy danych: mysql://root:pass@127.0.0.1:3306/gamedb
root - nazwa uzytkownika
pass - haslo
gamedb -nazwa bazy danych
Odpalamy konsole tak, aby znajdowala sie w glównym folderze aplikacji, uruchamiamy aplikacje komenda: cargo run

endpoint:
POST
/action
np: http://127.0.0.1:8000/action

przykladowe Body
{
"userId": 74,
"gameId": 45,
"action": "jump"
}

Headers musi zawierac:
content-type application/json




