package models

import (
    "common"
)

// {ModelName} Model说明
type {ModelName} struct {
    ID              int    `json:"id" xorm:"id pk"`  // 编号
    {ModelFields}
    *common.Model `xorm:"-"`
}

// {ModelNames} 数据访问对象
var {ModelNames} = {ModelName} {
    Model: &common.Model{
        TabName: "{TableName}",
    },
}

// TableName 指定数据表名
func (ths *{ModelName}) TableName() string {
    return "{TableName}"
}