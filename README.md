# To run docker
sudo systemctl start docker
sudo systemctl enable docker
sudo usermod -aG docker $USER


# To mirgrate database
Remember to set the DATABASE_URL in .env
cargo sqlx migrate run
