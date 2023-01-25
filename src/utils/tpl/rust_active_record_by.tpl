    /// 依据字段获取分页记录 - find_by_{FieldName}
    pub async fn find_by_{FieldName}(&self, pool: &Pool, val: &{FieldType}, query: &QueryBuilder) -> Vec<{StructName}> {
        let mut db = DB::table("{TableName}");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("{FieldName} = $1")
            .bind(val);
        db.find::<{StructName}>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_{FieldName}
    pub async fn find_all_by_{FieldName}(&self, pool: &Pool, val: &{FieldType}) -> Vec<{StructName}> {
        let mut db = DB::table("{TableName}");
        db.r#where("{FieldName} = $1").bind(val);
        db.find_all::<{StructName}>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_{FieldName}
    pub async fn get_by_{FieldName}(&self, pool: &Pool) -> Option<{StructName}> {
        let mut db = DB::table("{TableName}");
        db.r#where("{FieldName} = $1")
            .bind(&self.{FieldName})
            .limit(1);
        db.get::<{StructName}>(pool).await
    }