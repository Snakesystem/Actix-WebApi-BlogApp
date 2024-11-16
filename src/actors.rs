use crate::db_models:: { User, Article };
use crate::db_utils::DbActor;
use crate::schema::users::dsl::*;
use crate::schema::articles::{ dsl::*, id as article_id };
use crate::messages:: { FetchUser, FetchUserArticles, CreateArticleUser };
use crate::insertable::NewArticle;
use actix::Handler;
use diesel:: { self, prelude::* };

impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = self.0.get().expect("Cannot get connection from pool");

        users.get_results::<User>(&mut conn)
    }
}

impl Handler<FetchUserArticles> for DbActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, msg: FetchUserArticles, ctx: &mut Self::Context) -> Self::Result {
        let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = self.0.get().expect("Cannot get connection from pool");
        
        articles.filter(created_by.eq(msg.user_id)).get_results::<Article>(&mut conn)
    }
}

impl Handler<CreateArticleUser> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: CreateArticleUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = self.0.get().expect("Cannot get connection from pool");

        let new_article: NewArticle = NewArticle { title: msg.title, content: msg.content, created_by: msg.created_by };

        diesel::insert_into(articles).values(&new_article).returning((article_id, title, content, created_by, created_on.nullable())).get_result::<Article>(&mut conn)
    }
}