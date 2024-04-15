db.createUser({
    user: 'adminUser',
    pwd: 'G98c_33gsp=;2m4',
    roles: [
        {
            role: 'readWrite',
            db: 'admin',
        },
    ],
});

MonitoringDB = new Mongo().getDB("monitoringStorage");
MonitoringDB.createCollection('loginLogs', {capped: false, autoIndexId: false });
MonitoringDB.createCollection('registerLogs', {capped: false, autoIndexId: false });

UserStorageDB = new Mongo().getDB("userStorage");
UserStorageDB.createCollection('users', {capped: false, autoIndexId: false });
//UserStorageDB.createCollection('activeSessions', {capped: false, autoIndexId: false });

PostStorage = new Mongo().getDB("postStorage");
PostStorage.createCollection("boards", {capped: false, autoIndexId: false });
PostStorage.createCollection("threads", {capped: false, autoIndexId: false });
PostStorage.createCollection("posts", {capped: false, autoIndexId: false });