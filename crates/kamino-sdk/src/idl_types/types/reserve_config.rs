pub struct ReserveConfig {
  /** Status of the reserve Active/Obsolete/Hidden */
  pub status: u8,
  /** Asset tier -> 0 - regular (collateral & debt), 1 - isolated collateral, 2 - isolated debt */
  pub assetTier: u8,
  /** Flat rate that goes to the host */
  pub hostFixedInterestRateBps: u16,
  /**
   * [DEPRECATED] Space that used to hold 2 fields:
   * - Boost for side (debt or collateral)
   * - Reward points multiplier per obligation type
   * Can be re-used after making sure all underlying production account data is zeroed.
   */
  pub reserved2: Vec<u32>,
  /** Cut of the order execution bonus that the protocol receives, as a percentage */
  pub protocolOrderExecutionFeePct: u8,
  /** Protocol take rate is the amount borrowed interest protocol receives, as a percentage */
  pub protocolTakeRatePct: u8,
  /** Cut of the liquidation bonus that the protocol receives, as a percentage */
  pub protocolLiquidationFeePct: u8,
  /**
   * Target ratio of the value of borrows to deposits, as a percentage
   * 0 if use as collateral is disabled
   */
  pub loanToValuePct: u8,
  /** Loan to value ratio at which an obligation can be liquidated, as percentage */
  pub liquidationThresholdPct: u8,
  /** Minimum bonus a liquidator receives when repaying part of an unhealthy obligation, as bps */
  pub minLiquidationBonusBps: u16,
  /** Maximum bonus a liquidator receives when repaying part of an unhealthy obligation, as bps */
  pub maxLiquidationBonusBps: u16,
  /** Bad debt liquidation bonus for an undercollateralized obligation, as bps */
  pub badDebtLiquidationBonusBps: u16,
  /**
   * Time in seconds that must pass before redemptions are enabled after the deposit limit is
   * crossed.
   * Only relevant when `autodeleverage_enabled == 1`, and must not be 0 in such case.
   */
  pub deleveragingMarginCallPeriodSecs: u16,
  /**
   * The rate at which the deleveraging threshold decreases, in bps per day.
   * Only relevant when `autodeleverage_enabled == 1`, and must not be 0 in such case.
   */
  pub deleveragingThresholdDecreaseBpsPerDay: u16,
  /** Program owner fees assessed, separate from gains due to interest accrual */
  pub fees: types.ReserveFeesFields,
  /** Borrow rate curve based on utilization */
  pub borrowRateCurve: types.BorrowRateCurveFields,
  /** Borrow factor in percentage - used for risk adjustment */
  pub borrowFactorPct: u8,
  /** Maximum deposit limit of liquidity in native units, u64::MAX for inf */
  pub depositLimit: u64,
  /** Maximum amount borrowed, u64::MAX for inf, 0 to disable borrows (protected deposits) */
  pub borrowLimit: u64,
  /** Token id from TokenInfos struct */
  pub tokenInfo: types.TokenInfoFields,
  /** Deposit withdrawal caps - deposit & redeem */
  pub depositWithdrawalCap: types.WithdrawalCapsFields,
  /** Debt withdrawal caps - borrow & repay */
  pub debtWithdrawalCap: types.WithdrawalCapsFields,
  pub elevationGroups: Vec<u32>,
  pub disableUsageAsCollOutsideEmode: u8,
  /** Utilization (in percentage) above which borrowing is blocked. 0 to disable. */
  pub utilizationLimitBlockBorrowingAbovePct: u8,
  /**
   * Whether this reserve should be subject to auto-deleveraging after deposit or borrow limit is
   * crossed.
   * Besides this flag, the lending market's flag also needs to be enabled (logical `AND`).
   * **NOTE:** the manual "target LTV" deleveraging (enabled by the risk council for individual
   * obligations) is NOT affected by this flag.
   */
  pub autodeleverageEnabled: u8,
  pub reserved1: Vec<u64>,
  /**
   * Maximum amount liquidity of this reserve borrowed outside all elevation groups
   * - u64::MAX for inf
   * - 0 to disable borrows outside elevation groups
   */
  pub borrowLimitOutsideElevationGroup: u64,
  /**
   * Defines the maximum amount (in lamports of elevation group debt asset)
   * that can be borrowed when this reserve is used as collateral.
   * - u64::MAX for inf
   * - 0 to disable borrows in this elevation group (expected value for the debt asset)
   */
  pub borrowLimitAgainstThisCollateralInElevationGroup: Vec<u64>,
  /**
   * The rate at which the deleveraging-related liquidation bonus increases, in bps per day.
   * Only relevant when `autodeleverage_enabled == 1`, and must not be 0 in such case.
   */
  pub deleveragingBonusIncreaseBpsPerDay: u16
}