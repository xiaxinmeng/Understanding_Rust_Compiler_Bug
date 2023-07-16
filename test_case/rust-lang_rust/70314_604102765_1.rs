sh
docker run \
	--name mysql \
	--publish 127.0.0.1:3306:3306 \
	--env MYSQL_USER=refinery \
	--env MYSQL_PASSWORD=root \
	--env MYSQL_DATABASE=refinery_test \
	--env MYSQL_ROOT_PASSWORD=root \
	--detach \
	mysql:latest
