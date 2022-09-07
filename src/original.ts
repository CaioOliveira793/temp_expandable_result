
interface CollaboratorResult {
	id: string;
	name: string;
}

/// Expansion system

interface ExpandableResult<in T> {
	getResultId(): string;
	setResult(collab: T): void;
}

function collaboratorResultFromId(id: string): CollaboratorResult {
	return { id, name: 'fake' };
}

function collaboratorExpander(data: ExpandableResult<CollaboratorResult>): ExpandableResult<CollaboratorResult> {
	const collab = collaboratorResultFromId(data.getResultId());
	data.setResult(collab);
	return data;
}

/// Expansion Integration

interface ObservationResult {
	id: string;
	message: string;
	ownerId: string;

	// Expaned:
	owner?: CollaboratorResult;
}

class ObservationExpandableResult {
	constructor(private readonly data: ObservationResult) { }

	getOwnerExpansor(): ExpandableResult<CollaboratorResult> {
		return {
			getResultId: () => this.data.ownerId,
			setResult: (collab: CollaboratorResult) => this.data.owner = collab,
		}
	}
}
