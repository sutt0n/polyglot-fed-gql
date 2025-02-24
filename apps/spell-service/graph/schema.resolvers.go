package graph

import (
	"apps/spell-service/graph/model"
	"context"
	"math/rand"
)

// CastSpell is the resolver for the castSpell field.
func (r *mutationResolver) CastSpell(ctx context.Context, spell string, typeArg model.DamageType, playerID string) (*bool, error) {
	// ranodm damage between 1 and 10
	randomDmgFloat := rand.Float64() * 10

	spellToCast := model.CastedSpell{
		Spell:    spell,
		Type:     typeArg,
		PlayerID: playerID,
		Damage:   randomDmgFloat,
	}

	r.CastedSpells = append(r.CastedSpells, &spellToCast)

	r.mu.Lock()

  observer := r.SpellObservers[playerID]

  if observer != nil {
    observer <- &spellToCast
  }

	r.mu.Unlock()

	result := true

	return &result, nil
}

var baseSpells = []string{
    "fireball",
    "ice shard",
    "lightning bolt",
    "earthquake",
    "tornado",
}

// SpellBook is the resolver for the spellBook field.
func (r *queryResolver) SpellBook(ctx context.Context, playerID string) ([]*string, error) {
  spells := make([]*string, len(baseSpells))

  for i, spell := range baseSpells {
    spells[i] = &spell
  }

  return spells, nil
}

// SpellsCasted is the resolver for the spellsCasted field.
func (r *subscriptionResolver) SpellsCasted(ctx context.Context, target string) (<-chan *model.CastedSpell, error) {
	id := target
	spells := make(chan *model.CastedSpell, 1)

	go func() {
		<-ctx.Done()
		r.mu.Lock()
		delete(r.SpellObservers, id)
		r.mu.Unlock()
	}()

	r.mu.Lock()
	r.SpellObservers[id] = spells
	r.mu.Unlock()

	return spells, nil
}

// Mutation returns MutationResolver implementation.
func (r *Resolver) Mutation() MutationResolver { return &mutationResolver{r} }

// Query returns QueryResolver implementation.
func (r *Resolver) Query() QueryResolver { return &queryResolver{r} }

// Subscription returns SubscriptionResolver implementation.
func (r *Resolver) Subscription() SubscriptionResolver { return &subscriptionResolver{r} }

type mutationResolver struct{ *Resolver }
type queryResolver struct{ *Resolver }
type subscriptionResolver struct{ *Resolver }
