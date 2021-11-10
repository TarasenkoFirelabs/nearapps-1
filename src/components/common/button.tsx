import { Button as ButtonAnt } from 'antd'
import React from 'react'

export const Button = ({
  onClick,
  label,
  ...rest
}) => {
  return (
    <ButtonAnt {...rest} type="primary" onClick={onClick}>
      {label}
    </ButtonAnt>
  )
}

