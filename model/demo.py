from escrow import Escrow

e = Escrow({'client': 1000})
i = e.create('client', 'freelancer', 'arbitrator', 'ipfs://agreed-terms',
             [(100, 10), (200, 20)], ['client', 'freelancer', 'arbitrator'])
e.fund(i, 0, 'client')
e.submit(i, 0, 'ipfs://delivery', 'freelancer')
e.approve(i, 0, 'client')
e.fund(i, 1, 'client')
e.submit(i, 1, 'ipfs://partial-delivery', 'freelancer')
e.dispute(i, 1, 'client')
e.resolve(i, 1, 140, 60, 'arbitrator')
print('Milestones:', [m.status for m in e.engagements[i].milestones])
print('Balances:', e.balances)
print('Locked:', e.locked)
