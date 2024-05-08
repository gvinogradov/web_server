# File /docker-entrypoint-ext.sh it's ability
# to extend docker-entrypoint file in the
# specific web application in child Docker image.
if [ -f "/docker-entrypoint-ext.sh" ];
then
    . /docker-entrypoint-ext.sh
fi

./web_server