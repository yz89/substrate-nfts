// Copyright 2017-2019 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import React from 'react';
import BN from 'bn.js';
import { Button,InputNumber, TxButton } from '@polkadot/ui-app';
import { withCalls } from '@polkadot/ui-api/with';
import styled from 'styled-components';
import items from './items';
import {NFTokenId}from './types';
const ActionWrapper = styled.div`
  margin-top: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #e4e4e4;
  [type=radio] {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }
  

`;
type Props = {
  accountId?: string
};
type State = {
  lifetime: BN,
  radio_selection:string
};

class Owned extends React.PureComponent<Props> {
  state: State = {
    lifetime:new BN(10),
    radio_selection:'0'
  };
  onLifetimeChange = (lifetime: BN) => {
    this.setState({ lifetime });
  }

  _onRadioSelected(shortName:string, e:React.SyntheticEvent) {
    const updatedSelectedValues = e.target['value']
    //alert("selected_index: "+updatedSelectedValues)
  	this.setState({radio_selection: updatedSelectedValues})
  }
  render () {
    const { accountId } = this.props;
    const { lifetime,radio_selection } = this.state;
    const images = []
    for (var i=0;i<items.length;i++){
      if 
      images.push(
        <div key={i}>
        <label>
          <input type="radio" 
              			 name="radio1"
                     onChange={(e) => {this._onRadioSelected(i.toString(), e)}}
                     checked={this.state.radio_selection ===
                     					i.toString()}                     
                     value={i.toString()} />
        </label>
        </div>
      )
    }
    return (
      <section>
        <ActionWrapper>
        <div ><h3>NFTS</h3>
          <div style={{display:"flex"}}>{images}</div>
        </div>
        <div>
          <h3>Owned</h3>
          <InputNumber
              value={lifetime}
              label='lifetime'
              onChange={this.onLifetimeChange}
            />
          <Button.Group>
          <TxButton accountId={accountId}
          label='mint'
          params={[items[parseInt(radio_selection)].token_id,lifetime]}
          tx='nfts.mint'
          />
          </Button.Group>
        </div>
        </ActionWrapper>
      </section>
    );
  }
}
export default withCalls<Props>(
  ['query.nfts.nFTsCount', { propName: 'itemsCount' }]
)(Owned);