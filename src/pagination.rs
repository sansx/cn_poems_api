// https://github.com/diesel-rs/diesel/blob/v1.3.0/examples/postgres/advanced-blog-cli/src/pagination.rs
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use diesel::{associations::HasTable, pg::Pg};
use diesel::{prelude::*, query_dsl::methods::OffsetDsl};

// pub trait Paginate: Sized {
//     fn paginate(self, page: i64) -> Paginated<Self>;
// }

// impl<T> Paginate for T {
//     fn paginate(self, page: i64) -> Paginated<Self> {
//         Paginated {
//             query: self,
//             per_page: DEFAULT_PER_PAGE,
//             page,
//         }
//     }
// }

pub trait Paginate: AsQuery + Sized {
    fn paginate(self, page: i64) -> Paginated<Self::Query> {
      
        Paginated {
            query: self.as_query(),
            page,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl<T: AsQuery> Paginate for T {}

const DEFAULT_PER_PAGE: i64 = 10;

// pub struct Paginated<T> {
//   query: T,
//   page: i64,
//   per_page: i64,
// }

// impl Paginated<T> {
//   pub fn per_page(self, per_page: i64) -> Self {
//       Paginated { per_page, ..self }
//   }
// }

// const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
    }

    pub fn load_and_count_pages<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages))
    }
}

// impl<T: diesel::query_builder::AsQuery> Query for Paginated<T> {
//     type SqlType = (T::SqlType, BigInt);
// }

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

// impl<T: diesel::query_source::Table> Query for Paginated<T> {
//   type SqlType = (T::SqlType, BigInt);
// }

// impl<T: Query> OffsetDsl for Paginated<T> {
//     type Output = Paginated<T>;
//     fn offset(self, _: i64) -> <Self as OffsetDsl>::Output {
//         Paginated
//     }
// }

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}
