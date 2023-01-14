export interface Account {
  currency: Currency
}

export interface Currency {
  gold: number
  water: number
  purple: number
}

export type CurrencyType = keyof Currency

export interface ExchangeItem {
  id: number
  name: string
  price: number
  count: number
}
