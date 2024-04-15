use std::{
  cell::{Ref, RefCell, RefMut},
  collections::VecDeque,
  fmt,
  rc::Rc,
};

use anyhow::Context;
use uuid::Uuid;

use crate::{impl_common_getters, monitors::MonitorDto};

use super::{
  traits::{CommonGetters, PositionGetters},
  Container, ContainerType, DirectionContainer, TilingContainer,
  WindowContainer,
};

/// Root node of the container tree.
#[derive(Clone)]
pub struct RootContainer(Rc<RefCell<RootContainerInner>>);

struct RootContainerInner {
  id: Uuid,
  parent: Option<Container>,
  children: VecDeque<Container>,
  child_focus_order: VecDeque<Uuid>,
}

impl RootContainer {
  pub fn new() -> Self {
    let root = RootContainerInner {
      id: Uuid::new_v4(),
      parent: None,
      children: VecDeque::new(),
      child_focus_order: VecDeque::new(),
    };

    Self(Rc::new(RefCell::new(root)))
  }

  pub fn to_dto(&self) -> anyhow::Result<RootContainerDto> {
    let children = self
      .children()
      .iter()
      .map(|c| {
        c.as_monitor()
          .context("Root container has an invalid child type.")
          .and_then(|c| c.to_dto())
      })
      .try_collect()?;

    Ok(RootContainerDto {
      id: self.id(),
      parent: self.parent().map(|p| p.id()),
      children,
      child_focus_order: self.0.borrow().child_focus_order.clone().into(),
    })
  }
}

impl_common_getters!(RootContainer, ContainerType::Root);

impl PositionGetters for RootContainer {
  fn width(&self) -> anyhow::Result<i32> {
    Ok(0)
  }

  fn height(&self) -> anyhow::Result<i32> {
    Ok(0)
  }

  fn x(&self) -> anyhow::Result<i32> {
    Ok(0)
  }

  fn y(&self) -> anyhow::Result<i32> {
    Ok(0)
  }
}

impl fmt::Debug for RootContainer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(&self.to_dto().map_err(|_| std::fmt::Error), f)
  }
}

/// User-friendly representation of a root container.
///
/// Used for IPC and debug logging.
#[derive(Debug)]
pub struct RootContainerDto {
  id: Uuid,
  parent: Option<Uuid>,
  children: Vec<MonitorDto>,
  child_focus_order: Vec<Uuid>,
}
