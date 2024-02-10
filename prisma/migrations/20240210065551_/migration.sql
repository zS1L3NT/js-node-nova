-- CreateTable
CREATE TABLE "Config" (
    "filename" TEXT NOT NULL PRIMARY KEY,
    "shorthand" TEXT NOT NULL,
    "content" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "Secret" (
    "project" TEXT NOT NULL,
    "path" TEXT NOT NULL,
    "content" TEXT NOT NULL,

    PRIMARY KEY ("project", "path")
);
