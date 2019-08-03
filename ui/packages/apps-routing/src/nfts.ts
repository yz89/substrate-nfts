// Copyright 2017-2019 @polkadot/apps-routing authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import { Routes } from './types';

import App from '@polkadot/app-nfts';

export default ([
  {
    Component: App as any,
    display: {
      needsAccounts: true,
      needsApi: [
      ]
    },
    i18n: {
      defaultValue: 'nfts'
    },
    icon: 'th',
    name: 'nfts'
  }
] as Routes);
