use crate::execution_v1::physical_plan::physical_create_table::PhysicalCreateTable;
use crate::execution_v1::physical_plan::physical_projection::PhysicalProjection;
use crate::execution_v1::physical_plan::physical_table_scan::PhysicalTableScan;
use crate::execution_v1::physical_plan::PhysicalPlan;
use crate::planner::operator::scan::ScanOperator;
use crate::planner::operator::Operator;
use crate::planner::LogicalPlan;
use anyhow::anyhow;
use anyhow::Result;
use crate::execution_v1::physical_plan::physical_agg::PhysicalAgg;
use crate::execution_v1::physical_plan::physical_filter::PhysicalFilter;
use crate::execution_v1::physical_plan::physical_hash_join::PhysicalHashJoin;
use crate::execution_v1::physical_plan::physical_insert::PhysicalInsert;
use crate::execution_v1::physical_plan::physical_limit::PhysicalLimit;
use crate::execution_v1::physical_plan::physical_sort::PhysicalSort;
use crate::execution_v1::physical_plan::physical_values::PhysicalValues;
use crate::planner::operator::create_table::CreateTableOperator;
use crate::planner::operator::aggregate::AggregateOperator;
use crate::planner::operator::filter::FilterOperator;
use crate::planner::operator::insert::InsertOperator;
use crate::planner::operator::join::{JoinOperator, JoinType};
use crate::planner::operator::limit::LimitOperator;
use crate::planner::operator::project::ProjectOperator;
use crate::planner::operator::sort::SortOperator;
use crate::planner::operator::values::ValuesOperator;

pub struct PhysicalPlanBuilder { }

impl PhysicalPlanBuilder {
    pub fn new() -> Self {
        PhysicalPlanBuilder { }
    }

    pub fn build_plan(&mut self, plan: &LogicalPlan) -> Result<PhysicalPlan> {
        match &plan.operator {
            Operator::Project(op) => self.build_physical_select_projection(plan, op),
            Operator::Scan(scan) => Ok(self.build_physical_scan(scan.clone())),
            Operator::Filter(op) => self.build_physical_filter(plan, op),
            Operator::CreateTable(op) => Ok(self.build_physical_create_table(op)),
            Operator::Insert(op) => self.build_insert(plan, op),
            Operator::Values(op) => Ok(Self::build_values(op)),
            Operator::Sort(op) => self.build_physical_sort(plan, op),
            Operator::Limit(op) => self.build_physical_limit(plan, op),
            Operator::Join(op) => self.build_physical_join(plan, op),
            Operator::Limit(op)=>self.build_physical_limit(plan,op),
            Operator::Aggregate(op) => self.build_physical_agg(plan,op),
            _ => Err(anyhow!(format!(
                "Unsupported physical plan: {:?}",
                plan.operator
            ))),
        }
    }

    fn build_values(op: &ValuesOperator) -> PhysicalPlan {
        PhysicalPlan::Values(PhysicalValues { base: op.clone() })
    }

    fn build_insert(&mut self, plan: &LogicalPlan, op: &InsertOperator) -> Result<PhysicalPlan> {
        let input = self.build_plan(plan.child(0)?)?;

        Ok(PhysicalPlan::Insert(PhysicalInsert {
            table_name: op.table.clone(),
            input: Box::new(input),
        }))
    }

    fn build_physical_create_table(
        &mut self,
        op: &CreateTableOperator,
    ) -> PhysicalPlan {
        PhysicalPlan::CreateTable(
            PhysicalCreateTable {
                op: op.clone(),
            }
        )
    }

    fn build_physical_select_projection(&mut self, plan: &LogicalPlan, op: &ProjectOperator) -> Result<PhysicalPlan> {
        let input = self.build_plan(plan.child(0)?)?;

        Ok(PhysicalPlan::Projection(PhysicalProjection {
            exprs: op.columns.clone(),
            input: Box::new(input),
        }))
    }

    fn build_physical_scan(&mut self, base: ScanOperator) -> PhysicalPlan {
        PhysicalPlan::TableScan(PhysicalTableScan { base })
    }

    fn build_physical_filter(&mut self, plan: &LogicalPlan, base: &FilterOperator) -> Result<PhysicalPlan> {
        let input = self.build_plan(plan.child(0)?)?;

        Ok(PhysicalPlan::Filter(PhysicalFilter {
            predicate: base.predicate.clone(),
            input: Box::new(input),
        }))
    }

    fn build_physical_sort(&mut self, plan: &LogicalPlan, base: &SortOperator) -> Result<PhysicalPlan> {
        let input = self.build_plan(plan.child(0)?)?;

        Ok(PhysicalPlan::Sort(PhysicalSort {
            op: base.clone(),
            input: Box::new(input),
        }))
    }

    fn build_physical_limit(&mut self, plan: &LogicalPlan, base: &LimitOperator) -> Result<PhysicalPlan> {
        let input =self.build_plan(plan.child(0)?)?;

        Ok(PhysicalPlan::Limit(PhysicalLimit{
            op:base.clone(),
            input: Box::new(input),
        }))
    }

    fn build_physical_join(&mut self, plan: &LogicalPlan, base: &JoinOperator) -> Result<PhysicalPlan> {
        let left_input = Box::new(self.build_plan(plan.child(0)?)?);
        let right_input = Box::new(self.build_plan(plan.child(1)?)?);

        if base.join_type == JoinType::Cross {
            todo!()
        } else {
            Ok(PhysicalPlan::HashJoin(PhysicalHashJoin {
                op: base.clone(),
                left_input,
                right_input,
            }))
        }
    }
    fn build_physical_agg(&mut self, plan: &LogicalPlan, base: &AggregateOperator)->Result<PhysicalPlan>{
        let input =self.build_plan(plan.child(0)?)?;

        Ok(PhysicalPlan::Aggregate(PhysicalAgg{
            op:base.clone(),
            input: Box::new(input),
        }))
    }

}