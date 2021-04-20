echo "Starting server";

echo "Check: http://localhost:5000";

systemfd --no-pid -s http::5000 -- cargo watch -x run
