#include <stdio.h>
#include <stdlib.h>
#include "libpq-fe.h"

int
main(int argc, char **argv)
{
	PGconn        *conn;
	PGresult      *res;

	conn = PQconnectdb("");

	PQexec(conn, "DROP DOMAIN IF EXISTS schema_meta");
	PQexec(conn, "CREATE TYPE raw_schema_meta AS (version int4)");
	PQprepare(conn, "q1", "CREATE DOMAIN schema_meta AS raw_schema_meta CHECK ((value).version IS NOT NULL AND (value).version >= 1)", 0, NULL);
	PQexecPrepared(conn, "q1", 0, NULL, 0, 0, 0);
	PQexec(conn, "DROP DOMAIN IF EXISTS schema_meta");
	res = PQexecPrepared(conn, "q1", 0, NULL, 0, 0, 0);

	fprintf(stdout, "%s", PQresultErrorMessage(res));

	PQfinish(conn);

	return 0;
}
